// ExportBaseline.java - clean-room baseline exporter for the ghidra skill.
// @category GhidraSkill
//
// Exports seven baseline JSON files into the output directory given as the
// single script argument: functions, callgraph, types, vtables, constants,
// strings, imports. Uses Gson bundled with Ghidra. Writes only inside the
// provided output directory. Does NOT execute the target.
//
// Usage (headless): -postScript ExportBaseline.java <outputDir>
import java.io.FileWriter;
import java.io.IOException;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;

import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.data.DataType;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.FunctionManager;
import ghidra.program.model.listing.Data;
import ghidra.program.model.listing.DataIterator;
import ghidra.program.model.listing.Listing;
import ghidra.program.model.symbol.Reference;
import ghidra.program.model.symbol.ReferenceManager;
import ghidra.program.model.symbol.Symbol;
import ghidra.program.model.symbol.SymbolTable;
import ghidra.program.model.symbol.SymbolType;

public class ExportBaseline extends GhidraScript {

    private final Gson gson = new GsonBuilder().setPrettyPrinting().create();

    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args.length < 1) {
            printerr("ExportBaseline: missing output directory argument");
            return;
        }
        Path outDir = Paths.get(args[0]).toAbsolutePath().normalize();
        outDir.toFile().mkdirs();

        writeJson(outDir, "functions.json", exportFunctions());
        writeJson(outDir, "callgraph.json", exportCallgraph());
        writeJson(outDir, "types.json", exportTypes());
        writeJson(outDir, "vtables.json", exportVtables());
        writeJson(outDir, "constants.json", exportConstants());
        writeJson(outDir, "strings.json", exportStrings());
        writeJson(outDir, "imports.json", exportImports());
        println("ExportBaseline: wrote 7 baseline files to " + outDir);
    }

    private Object exportFunctions() {
        List<Object> funcs = new ArrayList<>();
        FunctionManager fm = currentProgram.getFunctionManager();
        for (Function f : fm.getFunctions(true)) {
            java.util.Map<String, Object> m = new java.util.LinkedHashMap<>();
            m.put("id", f.getEntryPoint().toString());
            m.put("name", f.getName());
            m.put("address", "0x" + f.getEntryPoint().toString());
            m.put("signature", f.getPrototypeString(false, false));
            m.put("thunk", f.isThunk());
            m.put("external", f.isExternal());
            funcs.add(m);
        }
        return wrap("functions", funcs);
    }

    private Object exportCallgraph() {
        List<Object> edges = new ArrayList<>();
        FunctionManager fm = currentProgram.getFunctionManager();
        for (Function f : fm.getFunctions(true)) {
            String caller = "0x" + f.getEntryPoint().toString();
            for (Function callee : f.getCalledFunctions(monitor)) {
                java.util.Map<String, Object> e = new java.util.LinkedHashMap<>();
                e.put("caller", caller);
                e.put("callee", "0x" + callee.getEntryPoint().toString());
                e.put("caller_name", f.getName());
                e.put("callee_name", callee.getName());
                edges.add(e);
            }
        }
        return wrap("callgraph", edges);
    }

    private Object exportTypes() {
        List<Object> types = new ArrayList<>();
        java.util.Iterator<DataType> it = currentProgram.getDataTypeManager().getAllDataTypes();
        while (it.hasNext()) {
            DataType dt = it.next();
            java.util.Map<String, Object> m = new java.util.LinkedHashMap<>();
            m.put("name", dt.getName());
            m.put("category", dt.getCategoryPath().getPath());
            m.put("length", dt.getLength());
            types.add(m);
        }
        return wrap("types", types);
    }

    private Object exportVtables() {
        // Candidate vtables: symbols whose name contains "vtable" or "vftable".
        List<Object> vtables = new ArrayList<>();
        SymbolTable st = currentProgram.getSymbolTable();
        for (Symbol s : st.getAllSymbols(true)) {
            String n = s.getName();
            if (n != null && (n.contains("vtable") || n.contains("vftable") || n.startsWith("_ZTV"))) {
                java.util.Map<String, Object> m = new java.util.LinkedHashMap<>();
                m.put("name", n);
                m.put("address", "0x" + s.getAddress().toString());
                m.put("namespace", s.getParentNamespace().getName(true));
                vtables.add(m);
            }
        }
        return wrap("vtables", vtables);
    }

    private Object exportConstants() {
        List<Object> constants = new ArrayList<>();
        SymbolTable st = currentProgram.getSymbolTable();
        for (Symbol s : st.getAllSymbols(true)) {
            if (s.getSymbolType() == SymbolType.LABEL && s.getSource().toString().equals("IMPORTED")) {
                continue;
            }
        }
        // Defined primitive data values act as recovered constants.
        Listing listing = currentProgram.getListing();
        DataIterator di = listing.getDefinedData(true);
        int cap = 5000;
        while (di.hasNext() && cap-- > 0) {
            Data d = di.next();
            if (d.isConstant() || d.hasStringValue()) {
                continue;
            }
            Object val = d.getValue();
            if (val != null && (val instanceof Number)) {
                java.util.Map<String, Object> m = new java.util.LinkedHashMap<>();
                m.put("address", "0x" + d.getAddress().toString());
                m.put("type", d.getDataType().getName());
                m.put("value", val.toString());
                constants.add(m);
            }
        }
        return wrap("constants", constants);
    }

    private Object exportStrings() {
        List<Object> strings = new ArrayList<>();
        Listing listing = currentProgram.getListing();
        DataIterator di = listing.getDefinedData(true);
        int cap = 20000;
        while (di.hasNext() && cap-- > 0) {
            Data d = di.next();
            if (d.hasStringValue()) {
                java.util.Map<String, Object> m = new java.util.LinkedHashMap<>();
                m.put("address", "0x" + d.getAddress().toString());
                Object v = d.getValue();
                m.put("value", v == null ? "" : v.toString());
                strings.add(m);
            }
        }
        return wrap("strings", strings);
    }

    private Object exportImports() {
        List<Object> imports = new ArrayList<>();
        SymbolTable st = currentProgram.getSymbolTable();
        for (Symbol s : st.getExternalSymbols()) {
            java.util.Map<String, Object> m = new java.util.LinkedHashMap<>();
            m.put("name", s.getName());
            m.put("namespace", s.getParentNamespace().getName(true));
            m.put("address", s.getAddress() != null ? "0x" + s.getAddress().toString() : null);
            imports.add(m);
        }
        return wrap("imports", imports);
    }

    private java.util.Map<String, Object> wrap(String key, List<Object> items) {
        java.util.Map<String, Object> doc = new java.util.LinkedHashMap<>();
        doc.put("schema_version", 1);
        doc.put("program", currentProgram.getName());
        doc.put("count", items.size());
        doc.put(key, items);
        return doc;
    }

    private void writeJson(Path dir, String name, Object obj) throws IOException {
        Path p = dir.resolve(name);
        try (FileWriter w = new FileWriter(p.toFile())) {
            w.write(gson.toJson(obj));
            w.write("\n");
        }
    }
}
