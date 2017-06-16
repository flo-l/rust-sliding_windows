var searchIndex = {};
searchIndex["sliding_windows"] = {"doc":"This crate provides an iterator adaptor that yields sliding windows into the elements of the wrapped Iterator.","items":[[3,"Storage","sliding_windows","This holds the backing allocation for the `Window` of a `Adaptor`.",null,null],[3,"Adaptor","","See sliding_windows for more information.",null,null],[3,"Window","","This is the `Item` type of the `Adaptor` iterator.",null,null],[3,"WindowIter","","",null,null],[3,"WindowIterMut","","",null,null],[11,"new","","Create a new `Storage` with a given window size. This will allocate twice as much memory as is needed to store the Window for performance reasons.",0,{"inputs":[{"name":"usize"}],"output":{"name":"storage"}}],[11,"from_vec","","Create a new `Storage` with a given window size from a given struct implementing `Into<Vec>`. The contents of the Vec will be removed. This will reuse the allocation of the Vec instead of allocating new memory.",0,{"inputs":[{"name":"s"},{"name":"usize"}],"output":{"name":"storage"}}],[11,"into","","",0,{"inputs":[{"name":"self"}],"output":{"name":"vec"}}],[11,"iter","","",1,{"inputs":[{"name":"self"}],"output":{"name":"windowiter"}}],[11,"iter_mut","","",1,{"inputs":[{"name":"self"}],"output":{"name":"windowitermut"}}],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"drop","","",1,{"inputs":[{"name":"self"}],"output":null}],[11,"eq","","",1,null],[11,"next","","",2,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"next","","",3,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"new","","This creates a new Adaptor. Usually you should be using",4,{"inputs":[{"name":"i"},{"name":"storage"}],"output":{"name":"adaptor"}}],[11,"next","","",4,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"size_hint","","",4,null],[8,"IterExt","","",null,null],[11,"sliding_windows","","",5,{"inputs":[{"name":"self"},{"name":"storage"}],"output":{"name":"adaptor"}}]],"paths":[[3,"Storage"],[3,"Window"],[3,"WindowIter"],[3,"WindowIterMut"],[3,"Adaptor"],[8,"IterExt"]]};
initSearch(searchIndex);