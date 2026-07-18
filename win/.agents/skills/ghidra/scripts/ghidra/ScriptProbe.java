// ScriptProbe.java - clean-room headless script probe for the skill.
// @category GhidraSkill
//
// Args: <outputJson>
// Writes a small JSON proving a headless script ran against the current program.
// Used by `ghidra script run` smoke tests. Read-only; does NOT execute target.
import java.io.FileWriter;
import java.nio.file.Path;
import java.nio.file.Paths;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;

import ghidra.app.script.GhidraScript;

public class ScriptProbe extends GhidraScript {

    private final Gson gson = new GsonBuilder().setPrettyPrinting().create();

    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        Path out = args.length >= 1 ? Paths.get(args[0]) : Paths.get("script-probe.json");
        java.util.Map<String, Object> doc = new java.util.LinkedHashMap<>();
        doc.put("schema_version", 1);
        doc.put("probe", "ok");
        doc.put("program", currentProgram != null ? currentProgram.getName() : null);
        doc.put("function_count",
                currentProgram != null
                        ? currentProgram.getFunctionManager().getFunctionCount() : 0);
        out.toAbsolutePath().getParent().toFile().mkdirs();
        try (FileWriter w = new FileWriter(out.toFile())) {
            w.write(gson.toJson(doc));
            w.write("\n");
        }
        println("ScriptProbe: wrote " + out);
    }
}
