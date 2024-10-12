# Diff
## /out.js
### esbuild
```js
// Users/user/project/node_modules/demo-pkg/index.js
var demo_pkg_default = exprWithSideEffects();

// Users/user/project/src/entry.js
console.log(demo_pkg_default);
```
### rolldown
```js

```
### diff
```diff
===================================================================
--- esbuild	/out.js
+++ rolldown	
@@ -1,2 +0,0 @@
-var demo_pkg_default = exprWithSideEffects();
-console.log(demo_pkg_default);

```