
 

interface callCommandResult {
    setOnData(ondata : (std)=>void);
    stopCommand();
} 


export interface UiAPi{
    sleep(n : number): Promise<void>;
    uiapi_path_join(str : string) : string;
    callUiApi(path : string, param : any) : Promise<string>;
    uiLog(str : string);
    callCommand(cmdname : string): Promise<callCommandResult>;
}