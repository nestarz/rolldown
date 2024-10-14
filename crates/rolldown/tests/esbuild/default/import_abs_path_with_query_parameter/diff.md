# Diff
## /out/entry.js
### esbuild
```js
// Users/user/project/file.txt?foo
var file_default = "This is some text";

// Users/user/project/file.txt#bar
var file_default2 = "This is some text";

// Users/user/project/entry.js
console.log(file_default, file_default2);
```
### rolldown
```js
import { default as foo } from "/Users/user/project/file.txt?foo";
import { default as bar } from "/Users/user/project/file.txt#bar";

//#region entry.js
console.log(foo, bar);

//#endregion

```
### diff
```diff
===================================================================
--- esbuild	/out/entry.js
+++ rolldown	entry.js
@@ -1,3 +1,3 @@
-var file_default = "This is some text";
-var file_default2 = "This is some text";
-console.log(file_default, file_default2);
+import {default as foo} from "/Users/user/project/file.txt?foo";
+import {default as bar} from "/Users/user/project/file.txt#bar";
+console.log(foo, bar);

```