import { describe, test } from "bun:test"
import { Database } from "bun:sqlite"
import { __test as dbTest } from "./db.ts"

describe("migrations", () => {
	test("runs without error and is idempotent", () => {
		const db = new Database(":memory:")
		// Run migrations; fail test if an exception is thrown
		dbTest.migrate(db)
		// Run again to ensure idempotency (no exceptions)
		dbTest.migrate(db)
	})
})
