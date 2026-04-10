import {HttpClient, HttpParams} from '@angular/common/http';
import {inject, Injectable} from '@angular/core';
import {ActivityResponse, Check, CreateHabitRequest, Habit, HabitWithCheck} from './habit.model';
import { environment } from '../../environments/environment';

@Injectable({
    providedIn: 'root'
})
export class HabitService {
    private readonly apiUrl = environment.apiUrl;
    private http = inject(HttpClient);

    createHabit(request: CreateHabitRequest) {
        return this.http.post<Habit>(`${this.apiUrl}/habits`, request);
    }

    getHabits(date?: string) {
        const options = date ? { params: { date: new Date(date).toISOString() } } : {};
        return this.http.get<HabitWithCheck[]>(`${this.apiUrl}/habits`, options);
    }

    deleteHabit(id: string) {
        return this.http.delete<void>(`${this.apiUrl}/habits/${id}`);
    }

    addCheck(habitId: string, value: number, checked_at?: string) {
        const body = checked_at
            ? { value, checked_at: new Date(checked_at).toISOString() }
            : { value };
        return this.http.post<Check>(`${this.apiUrl}/habits/${habitId}/checks`, body);
    }

    resetCheck(habitId: string, date?: string) {
        const options = date
            ? { params: { date: new Date(date).toISOString() }}
            : {};
        return this.http.delete(`${this.apiUrl}/habits/${habitId}/checks`, options);
    }

    /**
     * @param from - YYYY-MM-DD
     * @param to - YYYY-MM-DD
     */
    getActivity(from: string, to: string) {
        const params = new HttpParams().set('from', from).set('to', to);
        return this.http.get<ActivityResponse>(`${this.apiUrl}/checks/activity`, { params });
    }
}