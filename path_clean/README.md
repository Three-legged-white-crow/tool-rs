### path_clean
---

*Delete files recursively while preserving the directory structure.*


### build
---

- linux64: `make linux64`
- windows64: `make windows64`


### usage
---

- `./path_clean -p aim_path -v` will delete files of `aim_path` recursively while preserving the directory structure

  ```
  path_clean v1.0
  Delete files recursively while preserving the directory structure
  
  USAGE:
      path_clean [FLAGS] [OPTIONS]
  
  FLAGS:
      -h, --help       
              Prints help information
  
      -V, --version    
              Prints version information
  
      -v, --verbose    
              show detail of clean
  
  
  OPTIONS:
      -p, --path <aim dir>    
              directory that wait clean
  ```

