import {HttpClient} from '@angular/common/http';
import {inject, Injectable} from '@angular/core';
import {Check, CreateHabitRequest, Habit, HabitWithCheck} from './habit.model';

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

    addCheck(habitId: number, value: number, checked_at?: string) {
        let body = { value, checked_at: checked_at };
        return this.http.post<Check>(`${this.apiUrl}/habits/${habitId}/checks`, body);
    }

    resetCheck(habitId: number, date?: string) {
        const options = date ? { params: { date } } : {};
        return this.http.delete(`${this.apiUrl}/habits/${habitId}/checks`, options);
    }
}