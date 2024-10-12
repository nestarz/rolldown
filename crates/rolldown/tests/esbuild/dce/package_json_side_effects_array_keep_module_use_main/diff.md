# Diff
## /out.js
### esbuild
```js
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
@@ -1,1 +0,0 @@
-console.log("unused import");

```