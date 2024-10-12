# Diff
## /out.js
### esbuild
```js
// Users/user/project/src/index.js
var require_src = __commonJS({
  "Users/user/project/src/index.js"(exports, module) {
    module.exports = 123;
  }
});

// Users/user/project/src/dir/entry.js
console.log(require_src());
```
### rolldown
```js

```
### diff
```diff
===================================================================
--- esbuild	/out.js
+++ rolldown	
@@ -1,6 +0,0 @@
-var require_src = __commonJS({
-    "Users/user/project/src/index.js"(exports, module) {
-        module.exports = 123;
-    }
-});
-console.log(require_src());

```