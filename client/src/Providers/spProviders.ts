﻿import * as vscode from "vscode";
import * as glob from "glob";
import * as path from "path";
import { URI } from "vscode-uri";
import * as fs from "fs";
import * as spCompletions from "./spCompletions";
import { Include } from "./spCompletionsKinds";
import * as spDocCompletions from "./spDocCompletions";
import * as spDefinitions from "./spDefinitions";
import * as spParser from "./spParser";

export class Providers {
  completionsProvider: spCompletions.CompletionRepository;
  documentationProvider: spDocCompletions.JsDocCompletionProvider;
  definitionsProvider: spDefinitions.DefinitionRepository;
  hoverProvider: spCompletions.CompletionRepository;

  constructor(globalState?: vscode.Memento) {
    let CompletionRepo = new spCompletions.CompletionRepository(globalState);
    this.completionsProvider = CompletionRepo;
    this.definitionsProvider = new spDefinitions.DefinitionRepository(
      globalState
    );
    this.hoverProvider = CompletionRepo;
    this.documentationProvider = new spDocCompletions.JsDocCompletionProvider();
  }

  public handle_added_document(event: vscode.FileCreateEvent) {
    for (let file of event.files) {
      let Path = file.fsPath;
      Path = Path.replace(".git", "");
      this.completionsProvider.documents.set(path.basename(Path), file);
    }
  }

  public handle_document_change(event: vscode.TextDocumentChangeEvent) {
    let this_completions: spCompletions.FileCompletions = new spCompletions.FileCompletions(
      event.document.uri.toString()
    );
    let file_path: string = event.document.uri.fsPath;
    this.completionsProvider.documents.set(
      path.basename(file_path),
      event.document.uri
    );
    // Some file paths are appened with .git
    file_path = file_path.replace(".git", "");
    // We use parse_text here, otherwise, if the user didn't save the file, the changes wouldn't be registered.
    try {
      spParser.parse_text(
        event.document.getText(),
        file_path,
        this_completions,
        this.definitionsProvider.otherDefinitions,
        this.definitionsProvider.functionDefinitions,
        this.completionsProvider.documents
      );
    } catch (error) {
      console.log(error);
    }
    this.read_unscanned_imports(this_completions.includes);
    this.completionsProvider.completions.set(
      event.document.uri.toString(),
      this_completions
    );
  }

  public handle_new_document(document: vscode.TextDocument) {
    let this_completions: spCompletions.FileCompletions = new spCompletions.FileCompletions(
      document.uri.toString()
    );
    let file_path: string = document.uri.fsPath;
    // Some file paths are appened with .git
    if (file_path.includes(".git")) return;
    this.completionsProvider.documents.set(
      path.basename(file_path),
      document.uri
    );
    try {
      spParser.parse_file(
        file_path,
        this_completions,
        this.definitionsProvider.otherDefinitions,
        this.definitionsProvider.functionDefinitions,
        this.completionsProvider.documents
      );
    } catch (error) {
      console.log(error);
    }

    this.read_unscanned_imports(this_completions.includes);
    this.completionsProvider.completions.set(
      document.uri.toString(),
      this_completions
    );
  }

  public handle_document_opening(path: string) {
    let uri: string = URI.file(path).toString();
    let this_completions: spCompletions.FileCompletions = new spCompletions.FileCompletions(
      uri
    );
    // Some file paths are appened with .git
    path = path.replace(".git", "");
    try {
      spParser.parse_file(
        path,
        this_completions,
        this.definitionsProvider.otherDefinitions,
        this.definitionsProvider.functionDefinitions,
        this.completionsProvider.documents
      );
    } catch (error) {
      console.log(error);
    }

    this.read_unscanned_imports(this_completions.includes);
    this.completionsProvider.completions.set(uri, this_completions);
  }

  public read_unscanned_imports(includes: Include[]) {
    let debugSetting = vscode.workspace
      .getConfiguration("sourcepawn")
      .get("trace.server");
    let debug = debugSetting == "messages" || debugSetting == "verbose";
    for (let include of includes) {
      if (debug) console.log(include.uri.toString());
      let completion = this.completionsProvider.completions.get(include.uri);
      if (typeof completion === "undefined") {
        if (debug) console.log("reading", include.uri.toString());
        let file = URI.parse(include.uri).fsPath;
        if (fs.existsSync(file)) {
          if (debug) console.log("found", include.uri.toString());
          let new_completions: spCompletions.FileCompletions = new spCompletions.FileCompletions(
            include.uri
          );
          try {
            spParser.parse_file(
              file,
              new_completions,
              this.definitionsProvider.otherDefinitions,
              this.definitionsProvider.functionDefinitions,
              this.completionsProvider.documents,
              include.IsBuiltIn
            );
          } catch (err) {
            console.error(err, include.uri.toString());
          }
          if (debug) console.log("parsed", include.uri.toString());
          this.completionsProvider.completions.set(
            include.uri,
            new_completions
          );
          if (debug) console.log("added", include.uri.toString());
          this.read_unscanned_imports(new_completions.includes);
        }
      }
    }
  }

  public parse_sm_api(SourcemodHome: string): void {
    if (!SourcemodHome) return;
    let files = glob.sync(path.join(SourcemodHome, "**/*.inc"));
    for (let file of files) {
      try {
        let completions = new spCompletions.FileCompletions(
          URI.file(file).toString()
        );
        spParser.parse_file(
          file,
          completions,
          this.definitionsProvider.otherDefinitions,
          this.definitionsProvider.functionDefinitions,
          this.completionsProvider.documents,
          true
        );

        let uri =
          "file://__sourcemod_builtin/" + path.relative(SourcemodHome, file);
        this.completionsProvider.completions.set(uri, completions);
      } catch (e) {
        console.error(e);
      }
    }
  }
}
