// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ActionItem } from "./ActionItem";

export type Question = { id: string, created: string, creator: string, pretty_id: string, summary: string, details: string | null, investigation: string, outcome: string | null, status: string, action_items: { [key: string]: ActionItem }, };