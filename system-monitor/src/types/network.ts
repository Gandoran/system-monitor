export interface NetworkData{
    download : number,
    upload : number,
    netHistoryDownload: number[],
    netHistoryUpload:number[],
    netTotalDown: number,
    netTotalUp: number,
    netInterface: string,
    netIp: string,
    netPing: number,
}