use krama_core::object::Object;
use krama_internal::test_eval;

test_eval!(
  should_fs_read_write,
  r#"
        const fs = @import("std:fs")
        const { assertEqual } = @import("std:assert")

        const filePath = "test.txt"
        const fileContent = "Hello, Krama!"

        fs.writeFile(filePath, fileContent)

        const content = fs.readFile(filePath)

        fs.rm(filePath)

        assertEqual(content, fileContent)
    "#,
  Object::Void
);

test_eval!(
  should_fs_mkdir_rmdir,
  r#"
        const fs = @import("std:fs")
        const { assert } = @import("std:assert")

        const dirPath = "test_dir"

        fs.mkdir(dirPath)
        assert(fs.isDirectory(dirPath))

        fs.rmdir(dirPath)
        assert(!fs.exists(dirPath))
    "#,
  Object::Void
);
