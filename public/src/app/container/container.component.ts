import {Component, Input, OnInit} from '@angular/core';
import {Container} from "../container";

@Component({
    selector: 'container',
    templateUrl: './container.component.html',
    styleUrls: ['./container.component.scss']
})
export class ContainerComponent implements OnInit {

    @Input() container: Container | undefined = undefined

    constructor() {
    }

    ngOnInit(): void {
    }
}
