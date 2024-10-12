# Diff
## /out.js
### esbuild
```js
// Users/user/project/src/index.js
var src_default = 123;

// Users/user/project/src/dir/entry.js
console.log(src_default);
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
-var src_default = 123;
-console.log(src_default);

```