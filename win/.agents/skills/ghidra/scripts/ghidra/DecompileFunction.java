// DecompileFunction.java - clean-room single-function decompiler for the skill.
// @category GhidraSkill
//
// Args: <functionAddress> <outputDir>
// Writes source.c and analysis.json into <outputDir>. Resolves the function by
// entry-point address. Does NOT execute the target.
import java.io.FileWriter;
import java.nio.file.Path;
import java.nio.file.Paths;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;

import ghidra.app.decompiler.DecompInterface;
import ghidra.app.decompiler.DecompileResults;
import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.Function;

public class DecompileFunction extends GhidraScript {

    private final Gson gson = new GsonBuilder().setPrettyPrinting().create();

    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args.length < 2) {
            printerr("DecompileFunction: usage <address> <outputDir>");
            return;
        }
        String addrArg = args[0].startsWith("0x") ? args[0].substring(2) : args[0];
        Path outDir = Paths.get(args[1]).toAbsolutePath().normalize();
        outDir.toFile().mkdirs();

        Address addr = currentProgram.getAddressFactory().getAddress(addrArg);
        if (addr == null) {
            printerr("DecompileFunction: invalid address " + args[0]);
            return;
        }
        Function fn = getFunctionContaining(addr);
        if (fn == null) {
            fn = currentProgram.getFunctionManager().getFunctionAt(addr);
        }
        if (fn == null) {
            printerr("DecompileFunction: no function at " + args[0]);
            return;
        }

        DecompInterface decomp = new DecompInterface();
        boolean opened = decomp.openProgram(currentProgram);
        if (!opened) {
            printerr("DecompileFunction: could not open program for decompilation");
            return;
        }
        try {
            DecompileResults res = decomp.decompileFunction(fn, 120, monitor);
            java.util.Map<String, Object> analysis = new java.util.LinkedHashMap<>();
            analysis.put("schema_version", 1);
            analysis.put("name", fn.getName());
            analysis.put("address", "0x" + fn.getEntryPoint().toString());
            analysis.put("signature", fn.getPrototypeString(false, false));
            boolean ok = res != null && res.decompileCompleted();
            analysis.put("decompiled", ok);
            if (ok) {
                String src = res.getDecompiledFunction().getC();
                try (FileWriter w = new FileWriter(outDir.resolve("source.c").toFile())) {
                    w.write(src);
                }
                analysis.put("source_file", "source.c");
            } else {
                analysis.put("error", res == null ? "null result" : res.getErrorMessage());
            }
            try (FileWriter w = new FileWriter(outDir.resolve("analysis.json").toFile())) {
                w.write(gson.toJson(analysis));
                w.write("\n");
            }
            println("DecompileFunction: " + fn.getName() + " ok=" + ok);
        } finally {
            decomp.dispose();
        }
    }
}
