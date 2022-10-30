import {Component, Input, OnInit} from '@angular/core';
import {ContainerGet} from "./container";
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";

@Component({
    selector: 'container',
    templateUrl: './container.component.html',
    styleUrls: ['./container.component.scss']
})
export class ContainerComponent implements OnInit {

    @Input() container: ContainerGet | undefined = undefined

    constructor() {
    }

    ngOnInit(): void {
    }
}
