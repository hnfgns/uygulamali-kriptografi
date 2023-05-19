# Simetrik Şifreleme

Danışman bir vakit sonra kralın huzuruna döndü ve şöyle dedi: "Kralım, gizlilik arzınız için size bir çözüm sunabilirim. İletişiminizi şifreleyerek, mektuplarınızın sadece Şirin tarafından okunmasını sağlayabiliriz."

Kral Ferhat, bu fikirden büyük bir heyecan duydu ve danışmanının yol göstericiliğiyle mektuplarını şifrelemeye başladı.

Ferhat, her mektubunu özenle kaleme aldıktan sonra, onu özel bir anahtarla şifreledi. Bu anahtar, sadece Ferhat ve Şirin'in bildiği bir sırdı. Mektuplar, bu gizli anahtarla şifrelendikten sonra, düşmanların ellerine geçse bile anlamsız birer kağıt parçası olacaklardı. Böylece yalnızca Şirin elindeki anahtarı kullanarak mektupların şifresini çözecekti.

Danışman bu tekniğe simektrik şifreleme adını verdi. Zira islemler asagida gosterildigi gibi simetrik gerceklesiyordu.

```
     +------------+                +------------+
     |   Ferhat   |                |   Şirin    |
     +------------+                +------------+
         |                                 |
         | Şifreleme(Anahtar)              | Şifre Çözme(Anahtar)  
         |                                 |
         |          +------------+         |         
         |          | Şifrelenmiş|         |       
         +--------> |   Mektup   | +------ ^
                    +------------+            
```

