import {Component, OnInit} from '@angular/core';
import {Observable} from "rxjs";
import {ContainerGet} from "../container/container";
import {HttpClient} from "@angular/common/http";

@Component({
    selector: 'container-grid',
    templateUrl: './container-grid.component.html',
    styleUrls: ['./container-grid.component.scss']
})
export class ContainerGridComponent implements OnInit {

    containers: Observable<ContainerGet[]> = new Observable<ContainerGet[]>()

    constructor(private http: HttpClient) {
    }

    ngOnInit(): void {
        this.containers = this.http.get<ContainerGet[]>("http://localhost:8080/containers");
    }
}
