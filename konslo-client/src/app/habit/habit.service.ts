import {HttpClient} from '@angular/common/http';
import {inject, Injectable} from '@angular/core';
import {Check, CreateCheckRequest, CreateHabitRequest, Habit, HabitWithCheck, UpdateCheckRequest} from './habit.model';

@Injectable({
    providedIn: 'root'
})
export class HabitService {
    private readonly apiUrl = 'http://127.0.0.1:3000';
    private http = inject(HttpClient);

    createHabit(request: CreateHabitRequest) {
        return this.http.post<Habit>(`${this.apiUrl}/habits`, request);
    }

    getTodayHabits() {
        return this.http.get<HabitWithCheck[]>(`${this.apiUrl}/habits`);
    }

    deleteHabit(id: number) {
        return this.http.delete<void>(`${this.apiUrl}/habits/${id}`);
    }

    createCheck(habitId: number, request: CreateCheckRequest) {
        return this.http.post<Check>(`${this.apiUrl}/habits/${habitId}/checks`, request);
    }

    updateCheck(habitId: number, checkId: number, request: UpdateCheckRequest) {
        return this.http.put<Check>(`${this.apiUrl}/habits/${habitId}/checks/${checkId}`, request)
    }
}