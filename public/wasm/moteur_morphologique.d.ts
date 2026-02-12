/* tslint:disable */
/* eslint-disable */

export function ajouter_derive_a_racine(c1: string, c2: string, c3: string, mot: string, schema: string): boolean;

export function ajouter_racine(c1: string, c2: string, c3: string): string;

export function ajouter_scheme(nom: string, pattern: string, description: string): boolean;

export function charger_racines_depuis_texte(contenu: string): number;

export function chercher_racine(c1: string, c2: string, c3: string): boolean;

export function exporter_donnees(): string;

export function generer_derive(c1: string, c2: string, c3: string, schema: string): string;

export function generer_et_stocker_derive(c1: string, c2: string, c3: string, schema: string): boolean;

export function generer_et_stocker_tous_derives(c1: string, c2: string, c3: string): number;

export function generer_tous_derives(c1: string, c2: string, c3: string): any;

export function importer_donnees(json: string): boolean;

export function init_app(): void;

export function obtenir_derives_stockes(c1: string, c2: string, c3: string): any;

export function obtenir_tous_schemes(): any;

export function obtenir_toutes_racines(): any;

export function supprimer_derive(c1: string, c2: string, c3: string, mot: string): boolean;

export function supprimer_racine(c1: string, c2: string, c3: string): boolean;

export function supprimer_scheme(nom: string): boolean;

export function valider_mot_derive(mot: string, c1: string, c2: string, c3: string): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly ajouter_derive_a_racine: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
    readonly ajouter_racine: (a: number, b: number, c: number) => [number, number];
    readonly ajouter_scheme: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
    readonly charger_racines_depuis_texte: (a: number, b: number) => number;
    readonly chercher_racine: (a: number, b: number, c: number) => number;
    readonly exporter_donnees: () => [number, number];
    readonly generer_derive: (a: number, b: number, c: number, d: number, e: number) => [number, number];
    readonly generer_et_stocker_derive: (a: number, b: number, c: number, d: number, e: number) => number;
    readonly generer_et_stocker_tous_derives: (a: number, b: number, c: number) => number;
    readonly generer_tous_derives: (a: number, b: number, c: number) => any;
    readonly importer_donnees: (a: number, b: number) => number;
    readonly obtenir_derives_stockes: (a: number, b: number, c: number) => any;
    readonly obtenir_tous_schemes: () => any;
    readonly obtenir_toutes_racines: () => any;
    readonly supprimer_derive: (a: number, b: number, c: number, d: number, e: number) => number;
    readonly supprimer_racine: (a: number, b: number, c: number) => number;
    readonly supprimer_scheme: (a: number, b: number) => number;
    readonly valider_mot_derive: (a: number, b: number, c: number, d: number, e: number) => any;
    readonly init_app: () => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
