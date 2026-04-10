import {Component, inject} from '@angular/core';
import {Router, RouterLink, RouterLinkActive} from "@angular/router";
import {AuthService} from "../../auth/auth.service";

@Component({
    selector: 'k-left-sidebar',
    imports: [
        RouterLink,
        RouterLinkActive
    ],
    templateUrl: './left-sidebar.component.html',
    styleUrl: './left-sidebar.component.css',
})
export class LeftSidebarComponent {
    private authService = inject(AuthService);
    private router = inject(Router);


    onLogout() {
        this.authService.logout().subscribe({
            next: () => this.router.navigate(['/login'])
        });
    }
}
