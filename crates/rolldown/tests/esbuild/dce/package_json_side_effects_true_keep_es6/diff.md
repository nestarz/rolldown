# Diff
## /out.js
### esbuild
```js
// Users/user/project/node_modules/demo-pkg/index.js
console.log("hello");

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
-console.log("hello");
-console.log("unused import");

```