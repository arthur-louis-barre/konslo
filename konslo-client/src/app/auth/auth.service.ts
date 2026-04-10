import {inject, Injectable} from "@angular/core";
import {environment} from "../../environments/environment";
import {HttpClient} from "@angular/common/http";

@Injectable({
    providedIn: 'root'
})
export class AuthService {
    private readonly apiUrl = environment.apiUrl;
    private http = inject(HttpClient);

    register(email: string, password: string) {
        const body = { email, password };
        return this.http.post(`${this.apiUrl}/auth/register`, body);
    }

    login(email: string, password: string) {
        const body = { email, password };
        return this.http.post(`${this.apiUrl}/auth/login`, body);
    }

    logout() {
        return this.http.post(`${this.apiUrl}/auth/logout`, {})
    }

    me() {
        return this.http.get(`${this.apiUrl}/auth/me`)
    }

}