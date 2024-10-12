# Diff
## /out.js
### esbuild
```js
// Users/user/project/node_modules/demo-pkg/index.js
var demo_pkg_exports = {};
__export(demo_pkg_exports, {
  foo: () => foo
});
var foo = 123;
console.log("hello");

// Users/user/project/src/entry.js
console.log(demo_pkg_exports);
```
### rolldown
```js

```
### diff
```diff
===================================================================
--- esbuild	/out.js
+++ rolldown	
@@ -1,7 +0,0 @@
-var demo_pkg_exports = {};
-__export(demo_pkg_exports, {
-    foo: () => foo
-});
-var foo = 123;
-console.log("hello");
-console.log(demo_pkg_exports);

```