use krama_core::object::Object;
use krama_internal::test_eval;

test_eval!(
  should_get_array_length,
  "[1, 2, 3].length",
  Object::Integer(3)
);

test_eval!(
  should_get_string_length,
  "\"hello\".length",
  Object::Integer(5)
);

test_eval!(
  should_fs_read_dir,
  r#"
        const fs = @import("std:fs")
        const { assertEqual } = @import("std:assert")

        fs.mkdir("test_dir_2")
        fs.writeFile("test_dir_2/test.txt", "hello")

        const files = fs.readDir("test_dir_2")
        assertEqual(files.length, 1)

        fs.rm("test_dir_2/test.txt")
        fs.rmdir("test_dir_2")
    "#,
  Object::Void
);

test_eval!(
  should_fs_is_file,
  r#"
        const fs = @import("std:fs")
        const { assert } = @import("std:assert")

        fs.writeFile("test_file.txt", "hello")
        assert(fs.isFile("test_file.txt"))
        assert(!fs.isDirectory("test_file.txt"))

        fs.rm("test_file.txt")
    "#,
  Object::Void
);
