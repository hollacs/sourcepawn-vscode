import {
  TextDocument,
  workspace as Workspace,
  Range,
  Diagnostic,
} from "vscode";

import { parse, SyntaxError } from "../Parser/cfgParser/cfgParser";
import { cfgDiagnostics } from "./Linter/compilerDiagnostics";

/**
 * Lint a Valve Key Value TextDocument object and add its diagnostics to the collection.
 * @param  {TextDocument} document    The document to lint.
 * @returns void
 */
export async function refreshCfgDiagnostics(document: TextDocument) {
  await null;

  // Check if the setting to activate the linter is set to true.
  const workspaceFolder = Workspace.getWorkspaceFolder(document.uri);
  const enableLinter = Workspace.getConfiguration(
    "sourcepawn",
    workspaceFolder
  ).get<boolean>("enableLinter");

  // Stop early if linter is disabled.
  if (!enableLinter || document.languageId !== "valve-kv") {
    cfgDiagnostics.set(document.uri, []);
    return;
  }
  cfgDiagnostics.delete(document.uri);
  try {
    parse(document.getText(), undefined);
  } catch (e) {
    if (e instanceof SyntaxError) {
      const range = new Range(
        e.location.start.line - 1,
        e.location.start.column - 1,
        e.location.end.line - 1,
        e.location.end.column - 1
      );

      const msg = e.name + " " + e.message;
      const diag = new Diagnostic(range, msg);
      cfgDiagnostics.set(document.uri, [diag]);
    }
  }
}
