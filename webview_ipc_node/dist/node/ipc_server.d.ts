import { Server } from 'net';
export type OnReceiveFullByte = (data: Uint8Array) => Promise<Uint8Array>;
export declare function createIpcServer(ipcName: string, callback: OnReceiveFullByte | null): Server;
