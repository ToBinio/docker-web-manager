import {Component, OnInit} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {BasicContainerPost} from "../container";

@Component({
    selector: 'container-add',
    templateUrl: './container-add.component.html',
    styleUrls: ['./container-add.component.scss']
})
export class ContainerAddComponent implements OnInit {

    name: string = ""

    constructor(private http: HttpClient) {
    }

    ngOnInit(): void {
    }

    addContainer() {

        const headers = { 'content-type': 'application/json'}
        const data: BasicContainerPost = {
            name: this.name
        }

        this.http.post("http://localhost:8080/container", JSON.stringify(data), {headers: headers}).subscribe();
    }
}
