import { LRLanguage, LanguageSupport } from "https://esm.sh/@codemirror/language@6";
//import { parser } from "./quanta.js";

export const quantaLanguage = undefined //LRLanguage.define({ parser });
export function quanta() { return new LanguageSupport(quantaLanguage); }
