// VerifyMetadata.java - clean-room metadata verifier for the skill.
// @category GhidraSkill
//
// Args: <applyRecordsDir> <verifyOutputJson>
// Reads the apply-record JSONs, re-reads the current program, and confirms that
// each applied rename/signature is present. Writes a verify JSON. Read-only.
import java.io.FileReader;
import java.io.FileWriter;
import java.nio.file.Files;
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

public class VerifyMetadata extends GhidraScript {

    private final Gson gson = new GsonBuilder().setPrettyPrinting().create();

    @Override
    @SuppressWarnings("unchecked")
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args.length < 2) {
            printerr("VerifyMetadata: usage <recordsDir> <verifyOut>");
            return;
        }
        Path recordsDir = Paths.get(args[0]);
        Path out = Paths.get(args[1]);

        List<Object> results = new ArrayList<>();
        int verified = 0, mismatched = 0;
        if (Files.isDirectory(recordsDir)) {
            for (Path p : Files.newDirectoryStream(recordsDir, "*-apply.json")) {
                Map<String, Object> doc;
                try (FileReader r = new FileReader(p.toFile())) {
                    doc = gson.fromJson(r, new TypeToken<Map<String, Object>>(){}.getType());
                }
                String group = String.valueOf(doc.get("group"));
                List<Map<String, Object>> applied = (List<Map<String, Object>>) doc.get("applied");
                if (applied == null) continue;
                for (Map<String, Object> e : applied) {
                    Map<String, Object> rec = new java.util.LinkedHashMap<>();
                    rec.put("group", group);
                    rec.put("entry", e);
                    boolean ok = verifyEntry(group, e);
                    rec.put("verified", ok);
                    if (ok) verified++; else mismatched++;
                    results.add(rec);
                }
            }
        }

        Map<String, Object> doc = new java.util.LinkedHashMap<>();
        doc.put("schema_version", 1);
        doc.put("verified", verified);
        doc.put("mismatched", mismatched);
        doc.put("results", results);
        out.toAbsolutePath().getParent().toFile().mkdirs();
        try (FileWriter w = new FileWriter(out.toFile())) {
            w.write(gson.toJson(doc));
            w.write("\n");
        }
        println("VerifyMetadata: verified=" + verified + " mismatched=" + mismatched);
    }

    private boolean verifyEntry(String group, Map<String, Object> e) {
        try {
            Address addr = addr(e.get("address"));
            Function fn = getFunctionContaining(addr);
            if (fn == null) fn = currentProgram.getFunctionManager().getFunctionAt(addr);
            if (fn == null) return false;
            if (group.equals("renames")) {
                return fn.getName().equals(String.valueOf(e.get("new_name")));
            }
            if (group.equals("signatures")) {
                String c = fn.getComment();
                return c != null && c.contains(String.valueOf(e.get("signature")));
            }
        } catch (Exception ex) {
            return false;
        }
        return false;
    }

    private Address addr(Object v) {
        String s = String.valueOf(v);
        if (s.startsWith("0x")) s = s.substring(2);
        return currentProgram.getAddressFactory().getAddress(s);
    }
}
