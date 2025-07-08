export interface PackageJson {
    name: string;
    description?: string;
    license?: string;
    version: string;
    type?: string;
    types?: string;
    engines?: Record<string, string>;
    module?: string;
    main?: string;
    exports?: Record<string, any>;
    imports?: Record<string, any>;
    repository?: Repository;
    bugs?: Bugs;
    homepage?: string;
    keywords?: string[];
    devDependencies?: Record<string, string>;
    dependencies?: Record<string, string>;
    scripts?: Record<string, string>;
    _id?: string;
    _integrity?: string;
    _resolved?: string;
    _from?: string;
    _nodeVersion?: string;
    _npmVersion?: string;
    dist?: Dist;
    _npmUser?: NpmUser;
    directories?: Record<string, any>;
    maintainers?: Maintainer[];
    _npmOperationalInternal?: Record<string, any>;
    _hasShrinkwrap?: boolean;
  }
  
  export interface Repository {
    type: string;
    url: string;
    directory?: string;
  }
  
  export interface Bugs {
    url: string;
  }
  
  export interface Dist {
    integrity: string;
    shasum: string;
    tarball: string;
    fileCount?: number;
    unpackedSize?: number;
    attestations?: Record<string, any>;
    signatures?: Record<string, any>[];
  }
  
  export interface NpmUser {
    name: string;
    email: string;
    actor?: Actor;
  }
  
  export interface Actor {
    name: string;
    email: string;
    type: string;
  }
  
  export interface Maintainer {
    name: string;
    email: string;
  }