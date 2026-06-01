
export interface FParam{
    cmd : string,
    params : any
}

export type FetchFun =  (p: FParam)=>Promise<string>  
export type FunBtn = (s : string)=>HTMLButtonElement;
export type FunInput = (s : string)=>HTMLInputElement;