import {
  CompletionItemKind,
  Range,
  CompletionItem,
  Hover,
  DocumentSymbol,
  SymbolKind,
  LocationLink,
  Location,
} from "vscode";
import { URI } from "vscode-uri";

import { SPItem } from "./spItems";
import { globalIdentifier } from "../../Misc/spConstants";
import { ConstantItem } from "./spConstantItem";
import { MethodItem } from "./spMethodItem";
import { FunctionItem } from "./spFunctionItem";
import { descriptionToMD } from "../../spUtils";
import { EnumStructItem } from "./spEnumStructItem";
import { MethodMapItem } from "./spMethodmapItem";

export class VariableItem implements SPItem {
  name: string;
  filePath: string;
  detail: string;
  kind = CompletionItemKind.Variable;
  description?: string;
  parent: SPItem | ConstantItem;
  range: Range;
  type: string;
  references: Location[];
  enumStructName: string;

  constructor(
    name: string,
    file: string,
    parent: SPItem | ConstantItem,
    range: Range,
    type: string,
    enumStruct: string,
    detail: string,
    description = ""
  ) {
    this.name = name;
    this.filePath = file;
    this.parent = parent;
    this.range = range;
    this.type = type;
    this.enumStructName = enumStruct;
    this.references = [];
    this.detail = detail;
    this.description = description;
  }

  toCompletionItem(
    lastFunc: MethodItem | FunctionItem | undefined,
    lastMMorES: MethodMapItem | EnumStructItem | undefined,
    override?: boolean
  ): CompletionItem | undefined {
    if (override) {
      return {
        label: this.name,
        kind: this.kind,
      };
    }

    if (lastFunc === undefined) {
      if (this.parent.name === globalIdentifier) {
        return {
          label: this.name,
          kind: this.kind,
        };
      }
      return undefined;
    }

    if (lastMMorES === undefined) {
      if (this.parent.name === lastFunc.name) {
        return {
          label: this.name,
          kind: this.kind,
        };
      }
    }
    lastFunc = lastFunc as MethodItem;
    if (
      this.parent.name === lastFunc.name &&
      lastFunc.parent.name === lastMMorES.name
    ) {
      return {
        label: this.name,
        kind: this.kind,
      };
    }

    return undefined;
  }

  toDefinitionItem(): LocationLink {
    return {
      targetRange: this.range,
      targetUri: URI.file(this.filePath),
    };
  }

  toSignature() {
    return undefined;
  }

  toHover(): Hover | undefined {
    if (this.detail === "") {
      return undefined;
    }
    return new Hover([
      { language: "sourcepawn", value: this.detail },
      descriptionToMD(this.description),
    ]);
  }

  toDocumentSymbol(): DocumentSymbol {
    return new DocumentSymbol(
      this.name,
      this.type,
      SymbolKind.Variable,
      this.range,
      this.range
    );
  }
}
