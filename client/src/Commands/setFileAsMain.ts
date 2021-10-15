import { workspace as Workspace, window, Uri } from "vscode";

export async function run(args: any) {
  let uri: Uri = args.document.uri;
  let workspaceFolder =
    args === undefined ? undefined : Workspace.getWorkspaceFolder(args);
  if (uri === undefined) {
    window.showErrorMessage("No file are selected");
    return 1;
  }
  Workspace.getConfiguration("sourcepawn", workspaceFolder).update(
    "MainPath",
    uri.fsPath
  );
  return 0;
}
