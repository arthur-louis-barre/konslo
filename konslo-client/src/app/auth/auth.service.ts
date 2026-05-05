import {inject, Injectable} from "@angular/core";
import {environment} from "../../environments/environment";
import {HttpClient} from "@angular/common/http";

@Injectable({
    providedIn: 'root'
})
export class AuthService {
    private readonly apiUrl = environment.apiUrl;
    private http = inject(HttpClient);

    register(username: string, password: string) {
        const body = { username, password };
        return this.http.post(`${this.apiUrl}/auth/register`, body);
    }

    login(username: string, password: string) {
        const body = { username, password };
        return this.http.post(`${this.apiUrl}/auth/login`, body);
    }

    loginAsDemo() {
        return this.http.post(`${this.apiUrl}/auth/demo`, {});
    }

    logout() {
        return this.http.post(`${this.apiUrl}/auth/logout`, {})
    }

    me() {
        return this.http.get(`${this.apiUrl}/auth/me`)
    }
}