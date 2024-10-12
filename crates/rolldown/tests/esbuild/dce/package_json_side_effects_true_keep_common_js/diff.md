# Diff
## /out.js
### esbuild
```js
// Users/user/project/node_modules/demo-pkg/index.js
var require_demo_pkg = __commonJS({
  "Users/user/project/node_modules/demo-pkg/index.js"(exports) {
    exports.foo = 123;
    console.log("hello");
  }
});

// Users/user/project/src/entry.js
var import_demo_pkg = __toESM(require_demo_pkg());
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
@@ -1,8 +0,0 @@
-var require_demo_pkg = __commonJS({
-    "Users/user/project/node_modules/demo-pkg/index.js"(exports) {
-        exports.foo = 123;
-        console.log("hello");
-    }
-});
-var import_demo_pkg = __toESM(require_demo_pkg());
-console.log("unused import");

```