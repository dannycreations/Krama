use krama_core::object::Object;
use krama_internal::test_eval;

test_eval!(
  eval_fs_read_and_write,
  r#"
        const fs = import("std:fs")
        const { assertEqual, assert } = import("std:assert")

        const filePath = "test.txt"
        const fileContent = "Hello, Krama!"

        fs.writeFile(filePath, fileContent)
        assert(fs.exists(filePath))
        assert(fs.isFile(filePath))
        assert(!fs.isDirectory(filePath))

        const content = fs.readFile(filePath)

        fs.rm(filePath)
        assert(!fs.exists(filePath))

        assertEqual(content, fileContent)
    "#,
  Object::Void
);

test_eval!(
  eval_fs_mkdir_and_rmdir,
  r#"
        const fs = import("std:fs")
        const { assert } = import("std:assert")

        const dirPath = "test_dir"

        fs.mkdir(dirPath)
        assert(fs.exists(dirPath))
        assert(fs.isDirectory(dirPath))
        assert(!fs.isFile(dirPath))

        fs.rmdir(dirPath)
        assert(!fs.exists(dirPath))
    "#,
  Object::Void
);

test_eval!(
  eval_fs_read_dir,
  r#"
        const fs = import("std:fs")
        const { assert, assertEqual } = import("std:assert")

        const dirPath = "test_read_dir"
        const filePath1 = "test_read_dir/file1.txt"
        const filePath2 = "test_read_dir/file2.txt"

        fs.mkdir(dirPath)
        assert(fs.isDirectory(dirPath))

        fs.writeFile(filePath1, "content1")
        assert(fs.isFile(filePath1))

        fs.writeFile(filePath2, "content2")
        assert(fs.isFile(filePath2))

        const entries = fs.readDir(dirPath)
        assertEqual(entries.length, 2)

        fs.rm(filePath1)
        fs.rm(filePath2)
        fs.rmdir(dirPath)
        assert(!fs.exists(dirPath))
    "#,
  Object::Void
);
