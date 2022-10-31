import {Component, OnInit} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {AddContainerWS, ContainerGet, MessageWS} from "../container";
import {environment} from "../../environments/environment";

@Component({
    selector: 'container-grid',
    templateUrl: './container-grid.component.html',
    styleUrls: ['./container-grid.component.scss']
})
export class ContainerGridComponent implements OnInit {

    containers: ContainerGet[] = []

    constructor(private http: HttpClient) {
    }

    ngOnInit(): void {

        let webSocket: WebSocket;

        if (environment.wsDomain == undefined) {
            webSocket = new WebSocket("ws://" + window.location.host + "/ws");
        } else {
            webSocket = new WebSocket(environment.wsDomain);
        }

        webSocket.addEventListener("open", () => {
            webSocket.addEventListener("message", (msg) => {
                let data = JSON.parse(msg.data) as MessageWS;

                switch (data.mode) {
                    case "new": {
                        let request: AddContainerWS = data.data;

                        this.containers.push({
                            name: request.name
                        })

                    }
                }
            })
        })
        this.http.get<ContainerGet[]>(environment.domain + "/containers").subscribe((data) => {
            this.containers = data;
        });
    }
}
