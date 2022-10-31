import {Component, OnInit} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Container, MessageWS, UpdateStateContainerWS} from "../container";
import {environment} from "../../environments/environment";

@Component({
    selector: 'container-grid',
    templateUrl: './container-grid.component.html',
    styleUrls: ['./container-grid.component.scss']
})
export class ContainerGridComponent implements OnInit {

    containers: Container[] = []

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
                        let response: Container = data.data;

                        this.containers.push(response)
                        break;
                    }
                    case "updateState": {
                        let response: UpdateStateContainerWS = data.data;

                        for (let container of this.containers) {
                            if (container.uuid != response.uuid) continue

                            container.state = response.state;
                            break;
                        }
                        break;
                    }
                }
            })
        })
        this.http.get<Container[]>(environment.domain + "/containers").subscribe((data) => {
            this.containers = data;
        });
    }
}
