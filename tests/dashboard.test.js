import { test } from "node:test";
import assert from "node:assert/strict";
import { rows } from "../web/dashboard.js";
test("invalid report rejects", () => assert.throws(() => rows({})));
