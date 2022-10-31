export interface BasicContainerPost {
    name: string
}

export interface MessageWS {
    mode: string
    data: any
}

export interface Container {
    name: string
    uuid: string
    state: ContainerState
}

export interface UpdateStateContainerWS {
    uuid: string
    state: ContainerState
}

export enum ContainerState {
    STATE1,
    STATE2,
    STATE3,
}

