
 

interface callCommandResult {
    setOnData(ondata : (std : Uint8Array)=>void);
    stopCommand();
}  

export interface UiAPi{
    sleep(n : number): Promise<void>;
    uiapi_path_join(str : string) : string;
    callUiApiCore(path : string, param : any) : Promise<Response>;
    callUiApiBuffer(path : string, param : any) : Promise<ArrayBuffer>;
    callUiApi(path : string, param : any) : Promise<string>;
    uiLog(str : string);
    callCommand(cmdname : string): Promise<callCommandResult>;
    isDelimiterPresent(arr : Uint8Array):boolean;
}