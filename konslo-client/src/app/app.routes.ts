import {Routes} from '@angular/router';
import {HabitListComponent} from './habit/habit-list/habit-list.component';
import {HabitFormComponent} from "./habit/habit-form/habit-form.component";
import {authGuard} from "./auth/auth.guard";
import {LoginComponent} from "./auth/login/login.component";
import {RegisterComponent} from "./auth/register/register.component";
import {AuthLayoutComponent} from "./auth/auth-layout/auth-layout.component";
import {AppShellComponent} from "./shared/app-shell/app-shell.component";

export const routes: Routes = [
    {path: '', redirectTo: 'habits', pathMatch: 'full'},

    {
        path: '', component: AppShellComponent, canActivate: [authGuard], children: [
            {path: 'habits', component: HabitListComponent},
            {path: 'new-habit', component: HabitFormComponent},
        ]
    },

    {
        path: '', component: AuthLayoutComponent, children: [
            {path: 'login', component: LoginComponent},
            {path: 'register', component: RegisterComponent}
        ]
    }
];
