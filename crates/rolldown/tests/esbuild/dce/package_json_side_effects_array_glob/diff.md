# Diff
## /out.js
### esbuild
```js
// Users/user/project/node_modules/demo-pkg/keep/this/file.js
console.log("this should be kept");
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
-console.log("this should be kept");

```