import { NgModule } from "@angular/core";
import { CommonModule } from "@angular/common";
import { DomSanitizer } from "@angular/platform-browser";
import { FormsModule, ReactiveFormsModule } from "@angular/forms";
import { RouterModule } from "@angular/router";
import { MdIconModule, MdIconRegistry, MdProgressBarModule, MdTooltipModule } from "@angular/material";
import { BrowserAnimationsModule } from "@angular/platform-browser/animations";
import { BreadcrumbsComponent } from "./breadcrumbs/breadcrumbs.component";
import { ChannelsComponent } from "./channels/channels.component";
import { CheckingInputComponent } from "./checking-input/checking-input.component";
import { CopyableComponent } from "./copyable/copyable.component";
import { IconComponent } from "./icon/icon.component";
import { PackageInfoComponent } from "./package-info/package-info.component";
import { PackageListComponent } from "./package-list/package-list.component";
import { ProgressBarComponent } from "./progress-bar/progress-bar.component";
import { PlatformIconComponent } from "./platform-icon/platform-icon.component";
import { TabsComponent } from "./tabs/TabsComponent";
import { TabComponent } from "./tabs/TabComponent";
import { FormProgressComponent } from "./form-progress/form-progress.component";

@NgModule({
  imports: [
    BrowserAnimationsModule,
    CommonModule,
    FormsModule,
    MdIconModule,
    MdProgressBarModule,
    MdTooltipModule,
    ReactiveFormsModule,
    RouterModule
  ],
  declarations: [
    BreadcrumbsComponent,
    ChannelsComponent,
    CheckingInputComponent,
    CopyableComponent,
    IconComponent,
    PackageInfoComponent,
    PackageListComponent,
    ProgressBarComponent,
    PlatformIconComponent,
    TabsComponent,
    TabComponent,
    FormProgressComponent
  ],
  exports: [
    BreadcrumbsComponent,
    ChannelsComponent,
    CheckingInputComponent,
    CopyableComponent,
    IconComponent,
    PackageInfoComponent,
    PackageListComponent,
    ProgressBarComponent,
    PlatformIconComponent,
    TabsComponent,
    TabComponent,
    FormProgressComponent
  ]
})
export class SharedModule {
  constructor(private mdIconRegistry: MdIconRegistry, private sanitizer: DomSanitizer) {
    mdIconRegistry.addSvgIconSet(
        sanitizer.bypassSecurityTrustResourceUrl("/assets/images/icons/all.svg")
    );
  }
}