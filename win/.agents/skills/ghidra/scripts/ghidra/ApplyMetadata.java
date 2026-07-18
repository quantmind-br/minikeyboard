// ApplyMetadata.java - clean-room metadata applier for the skill.
// @category GhidraSkill
//
// Args: <group> <metadataJsonPath> <applyRecordsDir>
// group is one of: renames | signatures | types
// Applies recorded metadata to the program and writes an apply record JSON
// into <applyRecordsDir>. Runs inside a transaction. Does NOT execute target.
import java.io.FileReader;
import java.io.FileWriter;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import com.google.gson.reflect.TypeToken;

import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.Function;
import ghidra.program.model.symbol.SourceType;

public class ApplyMetadata extends GhidraScript {

    private final Gson gson = new GsonBuilder().setPrettyPrinting().create();

    @Override
    @SuppressWarnings("unchecked")
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args.length < 3) {
            printerr("ApplyMetadata: usage <group> <json> <recordsDir>");
            return;
        }
        String group = args[0];
        Path json = Paths.get(args[1]);
        Path recordsDir = Paths.get(args[2]).toAbsolutePath().normalize();
        recordsDir.toFile().mkdirs();

        Map<String, Object> doc;
        try (FileReader r = new FileReader(json.toFile())) {
            doc = gson.fromJson(r, new TypeToken<Map<String, Object>>(){}.getType());
        }
        List<Map<String, Object>> entries = (List<Map<String, Object>>) doc.get(group);
        List<Object> applied = new ArrayList<>();
        List<Object> failed = new ArrayList<>();

        int tx = currentProgram.startTransaction("apply " + group);
        boolean commit = false;
        try {
            for (Map<String, Object> e : entries) {
                try {
                    if (group.equals("renames")) {
                        applyRename(e, applied);
                    } else if (group.equals("signatures")) {
                        applySignature(e, applied);
                    } else {
                        // types are recorded but require a parser; mark deferred.
                        Map<String, Object> rec = new java.util.LinkedHashMap<>(e);
                        rec.put("status", "deferred");
                        rec.put("reason", "type application requires an explicit parser");
                        failed.add(rec);
                    }
                } catch (Exception ex) {
                    Map<String, Object> rec = new java.util.LinkedHashMap<>(e);
                    rec.put("status", "failed");
                    rec.put("error", ex.getMessage());
                    failed.add(rec);
                }
            }
            commit = true;
        } finally {
            currentProgram.endTransaction(tx, commit);
        }

        Map<String, Object> record = new java.util.LinkedHashMap<>();
        record.put("schema_version", 1);
        record.put("group", group);
        record.put("applied", applied);
        record.put("failed", failed);
        try (FileWriter w = new FileWriter(recordsDir.resolve(group + "-apply.json").toFile())) {
            w.write(gson.toJson(record));
            w.write("\n");
        }
        println("ApplyMetadata: " + group + " applied=" + applied.size() + " failed=" + failed.size());
    }

    private void applyRename(Map<String, Object> e, List<Object> applied) throws Exception {
        Address addr = addr(e.get("address"));
        Function fn = getFunctionContaining(addr);
        if (fn == null) fn = currentProgram.getFunctionManager().getFunctionAt(addr);
        if (fn == null) throw new Exception("no function at " + e.get("address"));
        String newName = String.valueOf(e.get("new_name"));
        fn.setName(newName, SourceType.USER_DEFINED);
        Map<String, Object> rec = new java.util.LinkedHashMap<>(e);
        rec.put("status", "applied");
        applied.add(rec);
    }

    private void applySignature(Map<String, Object> e, List<Object> applied) throws Exception {
        Address addr = addr(e.get("address"));
        Function fn = getFunctionContaining(addr);
        if (fn == null) fn = currentProgram.getFunctionManager().getFunctionAt(addr);
        if (fn == null) throw new Exception("no function at " + e.get("address"));
        // Record the intended signature as a plate comment for verification; a
        // full FunctionSignatureParser application is a follow-up capability.
        String sig = String.valueOf(e.get("signature"));
        fn.setComment("skill-signature: " + sig);
        Map<String, Object> rec = new java.util.LinkedHashMap<>(e);
        rec.put("status", "applied");
        rec.put("applied_as", "plate-comment");
        applied.add(rec);
    }

    private Address addr(Object v) {
        String s = String.valueOf(v);
        if (s.startsWith("0x")) s = s.substring(2);
        return currentProgram.getAddressFactory().getAddress(s);
    }
}
