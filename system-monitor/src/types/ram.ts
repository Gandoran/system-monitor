export interface RamData{
    ramTotal: number,    
    ramUsed: number,
    ramAvailable: number,
    swapUsed: number,
    swapTotal: number,
    ramHistory: number[],
    swapHistory: number[],
}