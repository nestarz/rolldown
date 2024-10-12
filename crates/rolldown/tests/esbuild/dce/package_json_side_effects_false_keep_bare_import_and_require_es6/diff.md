# Diff
## /out.js
### esbuild
```js
// Users/user/project/node_modules/demo-pkg/index.js
var demo_pkg_exports = {};
__export(demo_pkg_exports, {
  foo: () => foo
});
var foo;
var init_demo_pkg = __esm({
  "Users/user/project/node_modules/demo-pkg/index.js"() {
    foo = 123;
    console.log("hello");
  }
});

// Users/user/project/src/entry.js
init_demo_pkg();
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
-var demo_pkg_exports = {};
-__export(demo_pkg_exports, {
-    foo: () => foo
-});
-var foo;
-var init_demo_pkg = __esm({
-    "Users/user/project/node_modules/demo-pkg/index.js"() {
-        foo = 123;
-        console.log("hello");
-    }
-});
-init_demo_pkg();
-console.log("unused import");

```