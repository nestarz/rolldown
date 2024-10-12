# Diff
## /out.js
### esbuild
```js
// Users/user/project/node_modules/d/index.js
var foo = 123;

// Users/user/project/node_modules/b/index.js
throw "keep this";

// Users/user/project/src/entry.js
console.log(foo);
```
### rolldown
```js

```
### diff
```diff
===================================================================
--- esbuild	/out.js
+++ rolldown	
@@ -1,3 +0,0 @@
-var foo = 123;
-throw "keep this";
-console.log(foo);

```