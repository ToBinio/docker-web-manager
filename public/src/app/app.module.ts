import {NgModule} from '@angular/core';
import {BrowserModule} from '@angular/platform-browser';

import {AppRoutingModule} from './app-routing.module';
import {AppComponent} from './app.component';
import {ContainerGridComponent} from './container-grid/container-grid.component';
import {ContainerComponent} from './container/container.component';
import {HttpClientModule} from "@angular/common/http";
import { ContainerAddComponent } from './container-add/container-add.component';
import {FormsModule} from "@angular/forms";

@NgModule({
    declarations: [
        AppComponent,
        ContainerGridComponent,
        ContainerComponent,
        ContainerAddComponent,
    ],
    imports: [
        BrowserModule,
        AppRoutingModule,
        HttpClientModule,
        FormsModule
    ],
    providers: [],
    bootstrap: [AppComponent]
})
export class AppModule {
}
