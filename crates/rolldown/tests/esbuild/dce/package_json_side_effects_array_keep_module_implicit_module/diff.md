# Diff
## /out.js
### esbuild
```js
// Users/user/project/node_modules/demo-pkg/index-module.js
console.log("this should be kept");

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
@@ -1,2 +0,0 @@
-console.log("this should be kept");
-console.log("unused import");

```