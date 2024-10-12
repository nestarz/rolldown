# Diff
## /out.js
### esbuild
```js
// Users/user/project/node_modules/demo-pkg/index-main.js
var index_main_exports = {};
__export(index_main_exports, {
  foo: () => foo
});
var foo;
var init_index_main = __esm({
  "Users/user/project/node_modules/demo-pkg/index-main.js"() {
    foo = 123;
    console.log("this should be kept");
  }
});

// Users/user/project/src/require-demo-pkg.js
init_index_main();

// Users/user/project/src/entry.js
console.log("unused import");
```
### rolldown
```js

```
### diff
```diff
===================================================================
--- esbuild	/out.js
+++ rolldown	
@@ -1,13 +0,0 @@
-var index_main_exports = {};
-__export(index_main_exports, {
-    foo: () => foo
-});
-var foo;
-var init_index_main = __esm({
-    "Users/user/project/node_modules/demo-pkg/index-main.js"() {
-        foo = 123;
-        console.log("this should be kept");
-    }
-});
-init_index_main();
-console.log("unused import");

```