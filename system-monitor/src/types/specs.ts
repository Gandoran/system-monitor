export interface OsInfo {
    name: string;
    kernelVersion: string;
    buildNumber: string;
    architecture: string;
}

export interface CpuSpecs {
    architecture: string;
    vendor: string;
    maxClockMhz: number;
    l3CacheMb: number;
}

export interface MoboInfo {
    vendor: string;
    model: string;
    biosVersion: string;
}

export interface RamSpecs {
    totalCapacityBytes: number;
    speedMts: number;
    formFactor: string;
    manufacturer: string;
    partNumber: string;
}

export interface DisplaySpecs {
    name: string;
    resolutionX: number;
    resolutionY: number;
    refreshRateHz: number;
}

export interface DiskSpecs {
    model: string;
    capacityBytes: number;
    mediaType: string;
    interfaceType: string,
}

export interface SystemSpecsPayload {
    os: OsInfo;
    cpu: CpuSpecs;
    mobo: MoboInfo;
    ram: RamSpecs;
    displays: DisplaySpecs[];
    disks: DiskSpecs[];
}